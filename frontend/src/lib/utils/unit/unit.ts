import type { Characteristics } from './characteristics'
import { demonVigor, type Spell } from './magic'
import { marksmanship } from './special'
import {
	heavyWeapon,
	shortBow,
	sword,
	type BalisticWeapons,
	type Special,
	type Weapons
} from './weapons'

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

export interface HenchMan extends Unit {
	rank: 'henchman'
}

export interface HeroeUnit extends Unit {
	rank: 'heroe'
}

export interface MercenaryUnit extends Unit {
	rank: 'mercenary'
}

export type AllUnit = HenchMan | HeroeUnit | MercenaryUnit

export const testUnit: HenchMan = {
	id: 'gobWarior',
	name: 'Gob Gob',
	type: 'Warrior',
	rank: 'henchman',
	ballisticWeapon: shortBow,
	weapons: [sword, heavyWeapon],
	magic: [demonVigor],
	skills: [marksmanship],
	experience: 2,
	characteristics: {
		movement: 4,
		weapon_skill: 3,
		ballistic_skill: 4,
		strength: 2,
		toughness: 3,
		wounds: 1,
		initiative: 3,
		attacks: 1,
		leadership: 6,
		armorSave: 0
	},
	price: 15,
	description: ''
}

export const testHeroe: HeroeUnit = {
	id: 'gobWarior',
	name: 'Gob Gob',
	type: 'Warrior',
	ballisticWeapon: shortBow,
	weapons: [sword, heavyWeapon],
	magic: [demonVigor],
	skills: [marksmanship],
	experience: 2,
	rank: 'heroe',
	characteristics: {
		movement: 4,
		weapon_skill: 3,
		ballistic_skill: 4,
		strength: 2,
		toughness: 3,
		wounds: 1,
		initiative: 3,
		attacks: 1,
		leadership: 6,
		armorSave: 0
	},
	price: 15,
	description: ''
}
