import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'

export type availableLang = {
	en?: string
	fr?: string
}

export type langUnion = keyof availableLang

export type tradObject = {
	[property: string]: availableLang
}

export const lang: Writable<langUnion> = writable('fr')
