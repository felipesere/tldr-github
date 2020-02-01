<script>
    import Repo from './repo/Repo.svelte';
    import {onMount} from 'svelte';
    import AddNewRepo from "./modals/AddNewRepo.svelte";
    import Error from './errors/Error.svelte'
    import Fab from "./atoms/Fab.svelte";

    let repos = [];

    const fetchRepos = async () => {
        const response = await fetch('/api/repos');
        repos = await response.json();
    };

    onMount(fetchRepos);

    let showAddRepo = false;
    const close = () => showAddRepo = false;
    const open = () => showAddRepo = true;

    const handleNewRepo = async () => {
        await fetchRepos();
    }
</script>

<div class="main">
    <Fab onClick={open} />
    <Error />
    {#if showAddRepo}
        <AddNewRepo on:close={close} on:new-repo-added={handleNewRepo}/>
    {/if}

    <div class="my-container">
        {#if repos.length === 0}
            <p class="text-center subtle">No repos added yet</p>
        {/if}
        <div class="grid">
            {#each repos as repo (repo.id) }
                <Repo repo={repo} on:repo-deleted={fetchRepos}/>
            {/each}
        </div>
    </div>
</div>

<style>
    .my-container {
        margin: 2rem auto;
        width: 90%;
    }
    .main {
        display: flex;
    }
</style>
