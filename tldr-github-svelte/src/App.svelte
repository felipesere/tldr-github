<script>
  import Repo from './Repo.svelte';
  import AddRepo from './AddRepo.svelte';
  const fetchRepos = (async () => {
      const response = await fetch('http://localhost:8080/api/repos');

      return await response.json();
  })()

</script>

<div class="grid">
  {#await fetchRepos }
    <p>...loading...</p>
  {:then repos}
    {#each repos as repo}
    <Repo repo={repo} />
    {/each}
  {/await}
  <AddRepo />
</div>
