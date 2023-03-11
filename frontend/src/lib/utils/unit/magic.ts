import type { availableLang } from '$lib/utils/translation/langStore'

export interface Spell {
	name: availableLang
	number: number
	effect: availableLang
	difficulty: number
}

export interface MagicType {
	name: availableLang
	description: availableLang
	spellsList: Spell[]
}

export const demonVigor: Spell = {
	name: { fr: 'Vigueur Démoniaque' },
	number: 1,
	effect: {
		fr: 'Tout Portepeste ou Nurgling dans un rayon de 8 ps du Maître voit sa sauvegarde démoniaque passer à 4+ jusqu’au début de son prochain tour.'
	},
	difficulty: 7
}

export const nurgle: MagicType = {
	name: { fr: 'Benediction de Nurgle' },
	description: { fr: 'Vive la pouriture' },
	spellsList: [demonVigor]
}
