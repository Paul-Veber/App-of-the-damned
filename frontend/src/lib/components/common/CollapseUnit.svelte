<div class="collapse overflow-visible collapse-plus w-full">
	<input type="checkbox" />
	<div class="flex">
		<div class="collapse-title !pl-0">
			<div>
				<slot name="title" />
			</div>
		</div>

	</div>
	<div class="m-0 p-0 collapse-content w-fit">
		<slot />
	</div>
</div>
