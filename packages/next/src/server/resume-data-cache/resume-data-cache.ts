import type { UseCacheCacheStore, FetchCacheStore } from './cache-store'

/**
 * An immutable version of the resume data cache.
 */
export interface RenderResumeDataCache {
  /**
   * The cache store for the "use cache" cache.
   */
  readonly cache: Omit<UseCacheCacheStore, 'set'>

  /**
   * The cache store for the fetch cache.
   */
  readonly fetch: Omit<FetchCacheStore, 'set'>
}

/**
 * A mutable version of the resume data cache.
 */
export interface PrerenderResumeDataCache {
  /**
   * The cache store for the "use cache" cache.
   */
  readonly cache: UseCacheCacheStore

  /**
   * The cache store for the fetch cache.
   */
  readonly fetch: FetchCacheStore
}

/**
 * Creates a new mutable resume data cache. This cache can be mutated and then
 * sealed to create an immutable version of the cache.
 */
export function createPrerenderResumeDataCache(): PrerenderResumeDataCache {
  return {
    cache: new Map(),
    fetch: new Map(),
  }
}
