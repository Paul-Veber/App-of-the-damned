<script lang="ts">
	import Collapse from '$lib/components/common/Collapse.svelte'
  import Lang from '$lib/components/common/Lang.svelte'
  import { testFaction, type List } from '$lib/utils/unit/list'
	import { demonVigor } from '$lib/utils/unit/magic'
	import { marksmanship } from '$lib/utils/unit/special'
	import { testHeroe, type HenchMan } from '$lib/utils/unit/unit'
	import { heavyWeapon, shortBow, sword } from '$lib/utils/unit/weapons'
	import Unit from './Unit.svelte'

	let unit: HenchMan = {
		id: 'gobWarior',
		name: 'Gob Gob',
		type: 'Warrior',
		rank: 'henchman',
		ballisticWeapon: shortBow,
		weapons: [sword, heavyWeapon],
		magic: [demonVigor],
		skills: [marksmanship],
		experience: 2,
		characteristics: {
			movement: 4,
			weapon_skill: 3,
			ballistic_skill: 4,
			strength: 2,
			toughness: 3,
			wounds: 1,
			initiative: 3,
			attacks: 1,
			leadership: 6,
			armorSave: 0
		},
		price: 15,
		description: ''
	}

	const list: List = {
		name: 'Test list Name',
		lore: '',
		budget: 500,
		faction: testFaction,
		unitList: { heroes: [testHeroe], henchMen: [unit] }
	}
</script>

<div class="prose">
  <div class="w-list">
    <h1 class="text-center text-primary">{list.name}</h1>
    <h2 class="ml-2 italic"><Lang data={list.faction.name} /></h2>
  </div>
  <div class="mx-1 w-list">
    <Collapse>
      <h4 slot="title" class="ml-3 w-full mt-0">Heroes</h4>
      <div class="">
        {#each list.unitList.heroes as heroe }
          <Unit unit={heroe} />
        {/each}
      </div>
    </Collapse>
    {#if list.unitList.henchMen}
    <Collapse>
      <h4 slot="title" class="ml-3 w-full mt-0">Henchmen</h4>
      <div class="">
        {#each list.unitList.henchMen as henChman}
          <Unit unit={henChman} /> 
        {/each}
      </div>
    </Collapse>
    {/if}
  </div>
</div>

<style>
  .w-list {
    width: 98vw;
  }
</style>
