<script lang="ts">
	import Collapse from '$lib/components/common/Collapse.svelte'
	import type { Weapons } from '$lib/utils/unit/weapons'
	import WeaponTitle from './WeaponTitle.svelte'
	import Lang from '$lib/components/common/Lang.svelte'
	import { weaponTrad } from '$lib/trad/weaponTrad'
	import type { Unit } from '$lib/utils/unit/unit'

	export let unit: Unit
	export let weapon: Weapons
</script>

<Collapse>
	<WeaponTitle {weapon} {unit} slot="title" />
	{#if weapon.special}
		<div class="mb-3 w-full">
			<h3 class="italic mb-1"><Lang data={weaponTrad.special} /></h3>
			{#each weapon.special as special}
				<div class="mb-1 w-full"><Lang data={special.name} /> : <Lang data={special.effect} /></div>
			{/each}
		</div>
	{/if}
	<h3 class="italic"><Lang data={weaponTrad.description} /></h3>
	<div><Lang data={weapon.description} /></div>
</Collapse>
