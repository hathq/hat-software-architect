// SPDX-License-Identifier: Apache-2.0
// This owner adapter reuses Coela's verifier; it is not another schema engine.
import assert from 'node:assert/strict'
import { readFileSync, mkdtempSync, writeFileSync, rmSync } from 'node:fs'
import { resolve, join } from 'node:path'
import { tmpdir } from 'node:os'
import { pathToFileURL } from 'node:url'
import { createHash } from 'node:crypto'
import { execFileSync } from 'node:child_process'
import { createRequire } from 'node:module'
const root = process.cwd(), engineering = process.env.ENGINEERING_SOURCE
const binary = process.env.ARCHITECT_BINARY
if (!engineering || !binary?.startsWith('/')) throw Error('Explicit verifier source and compiled worker required')
const { parseDefinition, inspectCoverage, compareDefinitions } = await import(pathToFileURL(join(engineering, 'src/lifecycle.mts')).href)
const require = createRequire(join(engineering, 'package.json'))
const { Ajv2020 } = require('ajv/dist/2020.js')
const read = (path: string) => readFileSync(join(root, path))
const definition = parseDefinition(read('integrations/review/lifecycle.json'))
const ajv = new Ajv2020({ strict: true, ownProperties: true })
const inputSchema = JSON.parse(read('schemas/review-architecture-boundary-input-v2.schema.json').toString())
const outputSchema = JSON.parse(read('schemas/review-architecture-boundary-output-v2.schema.json').toString())
ajv.addSchema(inputSchema)
const validateInput = ajv.getSchema(inputSchema.$id), validateOutput = ajv.compile(outputSchema)
const input = JSON.parse(read('tests/fixtures/review-v2.json').toString())
assert.ok(validateInput(input), JSON.stringify(validateInput.errors))
const temp = mkdtempSync(join(tmpdir(), 'hat-review-'))
try {
  const output = join(temp, 'output.json')
  execFileSync(binary, ['review-v2', join(root, 'tests/fixtures/review-v2.json'), join(root, 'tests/fixtures/selected-binding.json'), output],
    { timeout: 15000, maxBuffer: 65536, env: {} })
  const actual = JSON.parse(readFileSync(output, 'utf8'))
  assert.ok(validateOutput(actual), JSON.stringify(validateOutput.errors))
  const kinds = ['supported', 'missing', 'stale', 'failed', 'conflicting', 'unresolved', 'missing']
  const expected = { schema: outputSchema.$id, scope_ref: input.scope_ref, revision: 5,
    semantic: input.semantic, authority_granted: false, findings: input.claims.map((c: any, i: number) => ({
      subject_ref: c.subject_ref, stage: c.stage, kind: kinds[i],
      evidence_refs: c.evidence.filter((e: any) => e.stage === c.stage).map((e: any) => e.reference) })) }
  assert.deepEqual(actual, expected)
  const records = (value: any) => value.findings.map((f: any) => ({
    schema: value.schema, scope_ref: value.scope_ref, revision: value.revision,
    semantic: value.semantic, authority_granted: value.authority_granted,
    subject_ref: f.subject_ref, stage: f.stage, kind: f.kind,
    evidence_sha256: createHash('sha256').update(JSON.stringify(f.evidence_refs)).digest('hex') }))
  const observations = [{ interface: 'review-finding', records: records(expected) },
    { interface: 'review-finding', consumer: 'offline-cli', records: records(actual) }]
  const findings = inspectCoverage(definition, observations, (_owner: string, path: string) => read(path))
  assert.deepEqual(findings, [])
  const old = { ...definition, version: '0.0.0', interfaces: [] }
  const report = { complete: true, definitionVersion: definition.version,
    changes: compareDefinitions(old, definition), findings, records: actual.findings.length,
    nativeSemanticAcceptance: false, authorityGranted: false, remoteWrites: false }
  if (process.env.REVIEW_REPORT) writeFileSync(process.env.REVIEW_REPORT, JSON.stringify(report, null, 2) + '\n')
  console.log(JSON.stringify(report))
} finally { rmSync(temp, { recursive: true, force: true }) }
