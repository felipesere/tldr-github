<script>
  import Error from './errors/Error.svelte';
  import Repo from './Repo.svelte';
  import Repo2 from './repo/Repo2.svelte';
  import AddRepo from './AddRepo.svelte';
  import { onMount } from 'svelte';

  let repos = []

  const fetchRepos = async () => {
    const response = await fetch('/api/repos');
    repos = await response.json();
  }

  onMount(fetchRepos)
</script>

<section class="section">
  <div class="container">
    <Error />
    <h1 class="title">
      Welcome to TLDR Github
    </h1>
    <p class="subtitle">
    These are the repos you are currently tracking
    </p>
  </div>
</section>

<section class="section">
  <div class="container">
    <AddRepo on:new-repo-added={fetchRepos}/>
  </div>
</section>


<section class="section">
  <div class="container">
    {#if repos.length === 0}
      <p>No repos added yet</p>
    {/if}
    <div class="grid">
      {#each repos as repo (repo.id) }
        <Repo2 repo={repo}  on:repo-deleted={fetchRepos}/>
      {/each}
    </div>
  </div>
</section>
