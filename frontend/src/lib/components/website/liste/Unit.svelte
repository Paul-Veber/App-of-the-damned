<script lang="ts">
	import { demonVigor } from '$lib/utils/unit/magic'
	import Collapse from '$lib/components/common/Collapse.svelte'
	import SimpleTitle from '$lib/components/common/SimpleTitle.svelte'
	import { characteristics } from '$lib/utils/unit/characteristics'
	import { heavyWeapon, shortBow, sword } from '$lib/utils/unit/weapons'
	import Magic from './magic/Magic.svelte'
	import BallisticWeapons from './weapon/BallisticWeapons.svelte'
	import Weapons from './weapon/Weapons.svelte'
	import type { Unit } from '$lib/utils/unit/unit'
	import { marksmanship } from '$lib/utils/unit/special'

	let unit: Unit = {
    id:'gobWarior',
		name: 'Gob Gob',
		type: 'Warrior',
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
      armorSave: 0,
		},
		price: 15,
		description: ''
	}

	let equipedWeaponNumber = 0

	let equipedWeapon = unit.weapons
	let weaponList: HTMLInputElement[] | [] = []

	const disableWeapon = () => {
		if (weaponList.length !== 0) {
			weaponList.forEach((element) => {
				switch (true) {
					case element.value === 'true' && element.checked === false && equipedWeaponNumber > 0:
						element.disabled = true
						break
					case equipedWeaponNumber >= 2 && element.checked === false:
						element.disabled = true
						break
					case equipedWeaponNumber < 2 && element.value !== 'true':
						element.disabled = false
						break
					case equipedWeaponNumber === 0 && element.value === 'true':
						element.disabled = false
						break
				}
			})
		}
	}

	const equipeWeapon = (e: Event) => {
		const element = e.currentTarget as HTMLInputElement
		if (element.checked === true) {
			equipedWeaponNumber += element.value === 'true' ? 2 : 1
		} else {
			equipedWeaponNumber -= element.value === 'true' ? 2 : 1
		}
		disableWeapon()
		console.log(equipedWeapon, equipedWeaponNumber)
	}

	const weaponsCost =
		unit.weapons.map((weapon) => weapon.price).reduce((acc, price) => acc + price) ?? 0
	const fullPrice = unit.price + weaponsCost
</script>

<div class="card w-fit cardwidth bg-base-100 shadow-xl">
	<div class="card-body">
		<div class="flex justify-between">
			<div class="card-title">{unit.name}</div>
			<div>{fullPrice}Co</div>
		</div>
		<div>Exp: {unit.experience}</div>
		<div class="italic">{unit.type}</div>
		<div class="">
			<table class="table table-zebra w-full">
				<thead>
					<tr class="">
						{#each characteristics.fr as characteristic}
							<th class="">{characteristic}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					<tr class="">
						{#each Object.entries(unit.characteristics) as [name, value]}
							<td class="">{value ?? '-'}</td>
						{/each}
					</tr>
				</tbody>
			</table>
		</div>
		<div class="">
			<div class="font-bold">Skills:</div>
			<ul class="list-disc ml-8">
      {#if unit.skills}
				{#each unit.skills as skill}
					<li>{skill}</li>
				{/each}
      {/if}
			</ul>
		</div>
		<div class="">
			<div class="font-bold">Weapons:</div>
			{#if unit.weapons.length !== 0}
				{#each unit.weapons as weapon, index}
					<div class="flex flex-row">
						<Weapons {weapon} {unit} />
						<div class="self-center">
							<input
								type="checkbox"
								class="checkbox block"
								value={weapon.twoHands}
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
				<div class="font-bold">Ballistic weapons:</div>
				<BallisticWeapons weapon={unit.ballisticWeapon} />
			{/if}
		</div>
		<div class="">
			{#if unit.magic && unit.magic.length !== 0}
				{#each unit.magic as spell}
					<div class="font-bold">Spells :</div>
					<Magic {spell} />
				{/each}
			{/if}
		</div>
		<div>
			<Collapse>
				<SimpleTitle slot="title" text="Description" />
				<textarea name="" id="" cols="70" rows="10" />
			</Collapse>
		</div>
	</div>
</div>
