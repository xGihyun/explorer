<script lang="ts">
	import { files } from '$lib/stores';
</script>

<div>
	{#if $files}
		<div class="space-y-4">
			{#each $files as file, idx (idx)}
				<div class="flex flex-col">
					<span class="text-lg font-bold">{file.name}</span>
					<span class="text-base">{file.size.toFixed(3)} MB</span>
					<span class="text-base opacity-75">{file.path}</span>
				</div>
			{/each}
		</div>
	{/if}
</div>
