<script lang="ts">
	import Collapse from "$lib/components/common/Collapse.svelte"
	import Lang from "$lib/components/common/Lang.svelte"
  import type { Special } from "$lib/utils/unit/weapons"
	import SkillsTitle from "./SkillsTitle.svelte"

  export let skills: Special[]
</script>

<Collapse>
  <SkillsTitle slot="title" {skills} />
    	{#each skills as special}
				<div class="mb-1 w-full"><Lang data={special.name} /> : <Lang data={special.effect} /></div>
			{/each}
</Collapse>
