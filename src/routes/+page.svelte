<script lang="ts">
	import { SearchResults } from '$lib/components';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let drives: any[];
	let files: String[];

	onMount(async () => {
		drives = await invoke('get_disks');
	});

	async function getFiles(path: string) {
		files = await invoke('get_files_in_dir', { path });
	}
</script>

<div>
	{#if drives}
		<div class="flex gap-4">
			{#each drives as drive, idx (idx)}
				<button class="bg-gray-400 p-2" on:click={() => getFiles(drive)} value={drive}>
					{drive}
				</button>
			{/each}
		</div>
	{/if}

	{#if files}
		<div class="flex flex-col">
			{#each files as file, idx (idx)}
				<span>{file}</span>
			{/each}
		</div>
	{/if}
	<SearchResults />
</div>
