import type { Characteristics } from './characteristics'
import type { Spell } from './magic'
import type { BalisticWeapons, Special, Weapons } from './weapons'

export interface Unit {
	id: string
	name: string
	type: string
	ballisticWeapon: BalisticWeapons
	weapons: Weapons[]
	magic?: Spell[]
	skills?: Special[]
	experience: number
	characteristics: Characteristics
	price: number
	description: string
}
