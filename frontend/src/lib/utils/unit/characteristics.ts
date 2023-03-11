export interface Characteristics {
	movement: number
	weapon_skill: number
	ballistic_skill: number
	strength: number
	toughness: number
	wounds: number
	initiative: number
	attacks: number
	leadership: number
	armorSave: number
}
const characteristics_fr = ['M', 'CC', 'CT', 'F', 'E', 'PV', 'I', 'A', 'Cd', 'Svg'] as const

export const characteristics = {
	fr: characteristics_fr
}
