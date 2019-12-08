<script>
  import Repo from './Repo.svelte';
  import AddRepo from './AddRepo.svelte';
  import { onMount } from 'svelte';

  let repos = []

  const fetchRepos = async () => {
      console.log("fetching...");
      const response = await fetch('http://localhost:8080/api/repos');

      repos = await response.json();
  }

  onMount(fetchRepos)

</script>

<div class="grid">
  {#if repos.length === 0}
    <p>No repos added yet</p>
  {:else}
    {#each repos as repo, index}
      <Repo repo={repo} />
    {/each}
  {/if}
  <AddRepo on:new-repo-added={fetchRepos} />
</div>
