<script>
    import Repo from './repo/Repo.svelte';
    import {onMount} from 'svelte';
    import SideMenu from "./menu/SideMenu.svelte";
    import AddNewRepoModal from "./AddNewRepoModal.svelte";
    import Error from './errors/Error.svelte'

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
    <Error />
    <SideMenu onClickAdd={open}/>
    {#if showAddRepo}
        <AddNewRepoModal on:close={close} on:new-repo-added={handleNewRepo}/>
    {/if}

    <div class="container my-4">
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
    .main {
        display: flex;
    }

    .my-4 {
        margin-top: 3rem;
        margin-bottom: 3rem;
    }
</style>
