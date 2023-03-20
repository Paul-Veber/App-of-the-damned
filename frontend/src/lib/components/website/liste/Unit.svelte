<script lang="ts">
	import Collapse from '$lib/components/common/Collapse.svelte'
	import SimpleTitle from '$lib/components/common/SimpleTitle.svelte'
	import { characteristics } from '$lib/utils/unit/characteristics'
	import Magic from './magic/Magic.svelte'
	import BallisticWeapons from './weapon/BallisticWeapons.svelte'
	import Weapons from './weapon/Weapons.svelte'
	import type { AllUnit } from '$lib/utils/unit/unit'
	import { pageTranslation } from '$lib/utils/translation/pageTranslation'
	import Lang from '$lib/components/common/Lang.svelte'
	import Skills from './skills/Skills.svelte'

  export let unit: AllUnit

	let equipedWeaponNumber = 0

	let equipedWeapon = unit.weapons
	let weaponList: HTMLInputElement[] | [] = []

	const disableWeapon = () => {
		if (weaponList.length !== 0) {
			weaponList.forEach((element) => {
				switch (true) {
					case element.value === 'twoHands' && element.checked === false && equipedWeaponNumber > 0:
						element.disabled = true
						break
					case equipedWeaponNumber >= 2 && element.checked === false:
						element.disabled = true
						break
					case equipedWeaponNumber < 2 && element.value !== 'twoHands':
						element.disabled = false
						break
					case equipedWeaponNumber === 0 && element.value === 'twoHands':
						element.disabled = false
						break
				}
			})
		}
	}

	const equipeWeapon = (e: Event) => {
		const element = e.currentTarget as HTMLInputElement
		if (element.checked === true) {
			equipedWeaponNumber += element.value === 'twoHands' ? 2 : 1
		} else {
			equipedWeaponNumber -= element.value === 'twoHands' ? 2 : 1
		}
		disableWeapon()
		console.log(equipedWeapon, equipedWeaponNumber)
	}

	const weaponsCost =
		unit.weapons.map((weapon) => weapon.price).reduce((acc, price) => acc + price) ?? 0
	const fullPrice = unit.price + weaponsCost
</script>

<div class="card card-compact w-full bg-base-300 shadow-xl not-prose">
	<div class="card-body">
		<div class="flex justify-between">
			<div class="card-title text-accent">{unit.name}</div>
			<div>{fullPrice}<Lang data={pageTranslation.global.goldCoin.short} /></div>
		</div>
		<div>Exp: {unit.experience}</div>
		<div class="italic">{unit.type}</div>
		<div class="">
			<table class="table table-compact table-zebra w-full">
				<thead>
					<tr class="text-center">
						{#each characteristics.fr as characteristic}
							<th class="">{characteristic}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					<tr class="text-center">
						{#each Object.entries(unit.characteristics) as [name, value]}
							<td class="text-center">{(!value || value === 0 ) ? '-' : value}</td>
						{/each}
					</tr>
				</tbody>
			</table>
		</div>
		<div class="">
      {#if unit.skills}
          <Skills skills={unit.skills} />
      {/if}
		</div>
		<div class="">
			<div class="font-bold"><Lang data={pageTranslation.liste.weapons} />:</div>
			{#if unit.weapons.length !== 0}
				{#each unit.weapons as weapon, index}
					<div class="flex flex-row">
						<Weapons {weapon} {unit} />
						<div class="self-center">
							<input
								type="checkbox"
								class="checkbox block"
								value={weapon.wielding}
								on:change={(e) => equipeWeapon(e)}
								bind:this={weaponList[index]}
							/>
						</div>
					</div>
				{/each}
			{/if}
		</div>
		<div class="">
			{#if unit.ballisticWeapon}
				<div class="font-bold"><Lang data={pageTranslation.liste.ballisticWeapons} />:</div>
				<BallisticWeapons weapon={unit.ballisticWeapon} />
			{/if}
		</div>
		<div class="">
			{#if unit.magic && unit.magic.length !== 0}
				{#each unit.magic as spell}
					<div class="font-bold"><Lang data={pageTranslation.liste.spell} /> :</div>
					<Magic {spell} />
				{/each}
			{/if}
		</div>
		<div>
			<Collapse>
				<SimpleTitle slot="title" text="Description" />
				<textarea name="" id="" cols="46" rows="10" />
			</Collapse>
		</div>
	</div>
</div>
