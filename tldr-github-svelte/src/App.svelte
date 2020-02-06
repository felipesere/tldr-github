<script>
    import {onMount} from 'svelte';
    import Tailwind from "./Tailwind.svelte";
    import Repo from './repo/Repo.svelte';
    import Error from './errors/Error.svelte'
    import Fab from "./atoms/Fab.svelte";

    import {onInterval} from "./support/interval";

    let repos = [];

    const fetchRepos = async () => {
        const response = await fetch('/api/repos');
        repos = await response.json();
    };

    onMount(fetchRepos);
    onInterval(() => fetchRepos(), 30 * 1000);

    const handleNewRepo = async () => {
        await fetchRepos();
    }
</script>

<Tailwind/>
<div>
  <Fab on:new-repo-added={fetchRepos} />
    <Error/>

    <div class="px-20 py-10">
        {#if repos.length === 0}
            <p class="text-center subtle">No repos added yet</p>
        {/if}
        <div class="grid">
            {#each repos as repo (repo.id) }
                <Repo repo={repo} on:repo-deleted={fetchRepos} on:repo-updated={fetchRepos}/>
            {/each}
        </div>
    </div>
</div>

