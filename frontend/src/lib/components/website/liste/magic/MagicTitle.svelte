<script lang="ts">
	import type { Spell } from '$lib/utils/unit/magic'
	import SpellDifficulty from '$lib/components/icons/SpellDifficulty.svelte'
	import Lang from '$lib/components/common/Lang.svelte'

	export let spell: Spell
</script>

<div class="grid grid-cols-2">
	<span class="block"><Lang data={spell.name} /></span>
	<div class="block justify-self-center">
		<SpellDifficulty />
		<span class="inline-block ml-1">{spell.difficulty}</span>
	</div>
</div>
