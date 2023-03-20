<script lang="ts">
	import type { BalisticWeapons } from '$lib/utils/unit/weapons'
	import Range from '$lib/components/icons/Range.svelte'
	import Strength from '$lib/components/icons/Strength.svelte'
	import Lang from '$lib/components/common/Lang.svelte'
	import Special from './Special.svelte'

	export let weapon: BalisticWeapons
</script>

<div class="grid grid-cols-4 pl-2">
	<span class="block mr-4"><Lang data={weapon.name} /></span>
	<div class="block">
		{#if 'strength' in weapon}<Strength />
			<span class="inline-block ml-1">{weapon.strength}</span>
		{/if}
	</div>
	<div class="block ml-4">
		{#if 'range' in weapon}<Range />
			<span class="inline-block ml-1">{weapon.range}</span>
		{/if}
	</div>
	<div class="block ml-6">
		<Special specials={weapon.special} />
	</div>
</div>
