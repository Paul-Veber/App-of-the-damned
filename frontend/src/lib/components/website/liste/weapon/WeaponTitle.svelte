<script lang="ts">
	import type { Weapons } from '$lib/utils/unit/weapons'
	import Strength from '../../../../components/icons/Strength.svelte'
	import Lang from '$lib/components/common/Lang.svelte'
	import Special from './Special.svelte'
	import type { Unit } from '$lib/utils/unit/unit'

	export let unit: Unit
	export let weapon: Weapons
	const weaponStrengthBonus = weapon.bonus?.strength ?? 0
</script>

<div class="grid grid-cols-3">
	<span class="block"><Lang data={weapon.name} /></span>
	<div class="block">
		<Strength />
		<span class="inline-block ml-1">{unit.characteristics.strength + weaponStrengthBonus}</span>
	</div>

	<div class="block">
		<Special specials={weapon.special} />
	</div>
</div>
