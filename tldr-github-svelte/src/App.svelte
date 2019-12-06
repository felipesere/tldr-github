<script>
  import Repo from './Repo.svelte';
  import AddRepo from './AddRepo.svelte';
  export let repos;

function sawAnewRepo({detail}) {
    console.log(detail.name)
  }
</script>

<div class="grid">
  {#each repos as repo}
    <Repo repo={repo} />
  {/each}
  <AddRepo on:new-repo={sawAnewRepo} />
</div>
