<script lang="ts">
	import type { Spell } from '$lib/utils/unit/magic'
	import Collapse from '$lib/components/common/Collapse.svelte'
	import Lang from '$lib/components/common/Lang.svelte'
	import MagicTitle from './MagicTitle.svelte'

	export let spell: Spell
</script>

<Collapse>
	<MagicTitle {spell} slot="title" />
	<div class="mb-1"><Lang data={spell.effect} /></div>
</Collapse>
