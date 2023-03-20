<script lang="ts">
	import Lang from "$lib/components/common/Lang.svelte"
	import { pageTranslation } from "$lib/utils/translation/pageTranslation"
	import type { Special } from "$lib/utils/unit/weapons"

  export let skills: Special[]
</script>


<div class="">
	{#if skills}
    <span class="font-bold"><Lang data={pageTranslation.liste.skills} />: </span>
		{#each skills as skill, index}
			<span class="italic"
				><Lang data={skill.name} />
				{#if index !== skills.length - 1} - {/if}
			</span>{/each}
	{/if}
</div>
