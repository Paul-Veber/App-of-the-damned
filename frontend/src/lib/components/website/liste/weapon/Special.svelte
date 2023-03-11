<script lang="ts">
	import type { Weapons } from '$lib/utils/unit/weapons'
	import Lang from '$lib/components/common/Lang.svelte'

	export let specials: Weapons['special']
</script>

<div class="block justify-self-center">
	{#if specials}
		{#each specials as special, index}
			<span class="italic"
				><Lang data={special.name} />
				{#if index !== specials.length - 1} - {/if}
			</span>{/each}
	{/if}
</div>
