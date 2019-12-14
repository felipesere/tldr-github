<script>
  import Repo from './Repo.svelte';
  import AddRepo from './AddRepo.svelte';
  import { onMount } from 'svelte';

  let repos = []

  const fetchRepos = async () => {
    const response = await fetch('/api/repos');
    repos = await response.json();
  }

  onMount(fetchRepos)
</script>

{#if repos.length === 0}
  <p>No repos added yet</p>
{/if}
<div class="grid">
  {#each repos as repo, index}
    <Repo repo={repo}  on:repo-deleted={fetchRepos} />
  {/each}
  <AddRepo on:new-repo-added={fetchRepos}/>
</div>
