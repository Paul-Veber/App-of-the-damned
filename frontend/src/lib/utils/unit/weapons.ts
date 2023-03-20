import type { Characteristics } from './characteristics'
import type { availableLang } from '$lib/utils/translation/langStore'

export interface Special {
	name: availableLang
	effect: availableLang
}

export interface Item {
	id: string
	name: availableLang
	description: availableLang
	bonus?: Partial<Characteristics>
	price: number
	special?: Special[]
}

export interface Weapons extends Item {
	wielding?: 'twoHands' | 'oneHand'
	shield?: boolean
}

export interface BalisticWeapons extends Item {
	range: number
	strength: number
}

export const shortBow: BalisticWeapons = {
	id: 'shortBow',
	name: { fr: 'Arc court' },
	description: { fr: 'Petit arc bon marché' },
	strength: 4,
	range: 16,
	special: [
		{
			name: { fr: 'test' },
			effect: { fr: 'bla' }
		},
		{ name: { fr: 'bli' }, effect: { fr: 'blu' } }
	],
	price: 10
}

export const sword: Weapons = {
	id: 'sword',
	name: { fr: 'Épée' },
	description: { fr: 'Une arme civilisée pour des temps barbare' },
	special: [
		{
			name: { fr: 'Parade' },
			effect: {
				fr: "Les épées offrent un excellent compromis de défense et d’attaque et permettent de parer les coups. Lorsque l'adversaire jette les dés pour toucher, lancez 2D6. Si le résultat est supérieur à son meilleur jet, votre figurine a paré le coup et l’attaque est annulée. Une figurine ne peut pas parer une attaque ayant le double ou plus de sa propre Force : elle est trop puissante pour être bloquée"
			}
		}
	],
	price: 10
}

export const heavyWeapon: Weapons = {
	id: 'heavyWeapon',
	name: { fr: 'Arme Lourde' },
	description: { fr: 'ça fait mal' },
	special: [
		{ name: { fr: 'Frappe en dernier' }, effect: { fr: "l'unité frappe toujours en dernier" } }
	],
	bonus: { strength: 2 },
	price: 15,
	wielding: 'twoHands'
}

export const weaponList = {
	sword,
	heavyWeapon
}

export const ballisticWeaponList = {
	shortBow
}
