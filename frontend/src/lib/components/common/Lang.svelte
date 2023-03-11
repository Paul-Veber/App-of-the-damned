<script lang="ts">
	import { lang, type availableLang } from '$lib/utils/translation/langStore'

	export let data: availableLang
</script>

{data[$lang]}
