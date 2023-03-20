import type { availableLang } from '../translation/langStore'
import { testUnit, type HenchMan, type HeroeUnit, type MercenaryUnit, type Unit } from './unit'
import { heavyWeapon, sword, type Item, type Special } from './weapons'

export interface List {
	name: string
	faction: Faction
	lore: string
	budget: number
	unitList: UnitList
}

export interface UnitList {
	heroes: HeroeUnit[]
	henchMen?: HenchMan[]
	mercenaries?: MercenaryUnit[]
}

export interface Faction {
	name: availableLang
	units: Unit[]
	itemList: Item[]
	special: Special
}

export const testFaction: Faction = {
	name: {fr:'Test faction name'},
	units: [testUnit],
	itemList: [sword, heavyWeapon],
	special: { name: { fr: 'bla' }, effect: { fr: 'test' } }
}
