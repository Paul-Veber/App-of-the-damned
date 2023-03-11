<script lang="ts">
	import Collapse from '$lib/components/common/Collapse.svelte'
	import type { BalisticWeapons } from '$lib/utils/unit/weapons'
	import BallisticWeaponTitle from './BallisticWeaponTitle.svelte'
	import Lang from '$lib/components/common/Lang.svelte'
	import { weaponTrad } from '$lib/trad/weaponTrad'

	export let weapon: BalisticWeapons
</script>

<Collapse>
	<BallisticWeaponTitle {weapon} slot="title" />
	{#if weapon.special}
		<div class="mb-3">
			<h3 class="italic mb-1"><Lang data={weaponTrad.special} /></h3>
			{#each weapon.special as special}
				<div class="mb-1"><Lang data={special.name} /> : <Lang data={special.effect} /></div>
			{/each}
		</div>
	{/if}
	<h3 class="italic"><Lang data={weaponTrad.description} /></h3>
	<div><Lang data={weapon.description} /></div>
</Collapse>
