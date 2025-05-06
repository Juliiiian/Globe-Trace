import type { Writable } from 'svelte/store';

export type GetElementType<T extends any[]> = T extends (infer U)[] ? U : never;
export type GetRecordValue<T extends Record<any, any>> = T extends Record<any, infer U> ? U : never;
export type NonNullable<T> = Exclude<T, null | undefined>;
export type GetStoreType<T extends Writable<any>> = T extends Writable<infer U> ? U : never;
