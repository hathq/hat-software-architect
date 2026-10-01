import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile } from 'node:fs/promises'
import test from 'node:test'

const root = new URL('../authoring/', import.meta.url)
const manifest = JSON.parse(await readFile(
  new URL('authoring-kit-v1.json', root), 'utf8'))

test('authoring assets are closed, pinned, and do not upload or submit', async () => {
  assert.equal(manifest.schema,
    'hathq://hat-software-architect/authoring-kit/v1')
  assert.equal(manifest.runtime_discovery, false)
  assert.equal(manifest.browser_package_upload, false)
  assert.equal(manifest.implicit_submission, false)
  assert.equal(manifest.assets.length, 5)
  assert.equal(new Set(manifest.assets.map(value => value.role)).size, 5)
  for (const asset of manifest.assets) {
    assert.match(asset.sha256, /^[0-9a-f]{64}$/u)
    const bytes = await readFile(new URL(asset.path, root))
    assert.equal(createHash('sha256').update(bytes).digest('hex'), asset.sha256)
  }
})

test('development request is closed and keeps create and improve distinct', async () => {
  const schema = JSON.parse(await readFile(
    new URL('../schemas/hat-development-request-v1.schema.json', root), 'utf8'))
  assert.equal(schema.additionalProperties, false)
  assert.deepEqual(schema.properties.kind.enum, ['create', 'improve'])
  assert.deepEqual(schema.properties.target.oneOf[0], { type: 'null' })
  assert.equal(schema.properties.evidence_references.items.additionalProperties,
    false)
  assert.equal(schema.properties.evidence_references.items.properties.sha256.pattern,
    '^[0-9a-f]{64}$')
})

test('prompts forbid guessing and implicit publication', async () => {
  const create = await readFile(new URL('prompts/create-hat-v1.md', root), 'utf8')
  const improve = await readFile(new URL('prompts/improve-hat-v1.md', root), 'utf8')
  assert.match(create, /never guess/u)
  assert.match(create, /Do not publish, upload/u)
  assert.match(improve, /Do not infer/u)
  assert.match(improve, /Do not publish, upload/u)
})
