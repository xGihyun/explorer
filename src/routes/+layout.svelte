<script lang="ts">
	import '../app.postcss';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { FileData } from '$lib/types/files';
	import { files } from '$lib/stores';

	let query: string;
	let path: string;

	async function explore() {
		const result = (await invoke('explore', { path, query })) as FileData[];

		files.set(result);
	}
</script>

<input type="text" placeholder="Search this PC" bind:value={query} />
<input type="text" placeholder="Enter path" bind:value={path} />
<button on:click={explore}>Search!</button>
<slot />
