import type { RenderResumeDataCache } from './resume-data-cache'
import {
  parseUseCacheCacheStore,
  parseFetchCacheStore,
  stringifyFetchCacheStore,
  stringifyUseCacheCacheStore,
} from './cache-store'

type ResumeStoreSerialized = {
  store: {
    cache: {
      [key: string]: any
    }
    fetch: {
      [key: string]: any
    }
  }
}

/**
 * Serializes an immutable resume data cache into a JSON string.
 */
export async function stringifyResumeDataCache(
  resumeDataCache: RenderResumeDataCache
): Promise<string> {
  if (resumeDataCache.fetch.size === 0 && resumeDataCache.cache.size === 0) {
    return 'null'
  }

  const json: ResumeStoreSerialized = {
    store: {
      fetch: Object.fromEntries(
        stringifyFetchCacheStore(resumeDataCache.fetch.entries())
      ),
      cache: Object.fromEntries(
        await stringifyUseCacheCacheStore(resumeDataCache.cache.entries())
      ),
    },
  }

  return JSON.stringify(json)
}

/**
 * Parses a serialized resume data cache into an immutable version of the cache.
 * This cache cannot be mutated further, and is returned sealed.
 */
export function parseResumeDataCache(text: string): RenderResumeDataCache {
  if (text === 'null') {
    return {
      cache: new Map(),
      fetch: new Map(),
    }
  }

  const json: ResumeStoreSerialized = JSON.parse(text)
  return {
    cache: parseUseCacheCacheStore(Object.entries(json.store.cache)),
    fetch: parseFetchCacheStore(Object.entries(json.store.fetch)),
  }
}
