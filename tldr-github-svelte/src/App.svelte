<script>
    import Repo from './repo/Repo.svelte';
    import {onMount} from 'svelte';
    import AddNewRepo from "./modals/AddNewRepo.svelte";
    import Error from './errors/Error.svelte'
    import Fab from "./atoms/Fab.svelte";
    import Tailwind from "./Tailwind.svelte";
    import AddRepo from "./modals/AddRepo.svelte";
    import {onInterval} from "./support/interval";

    let repos = [];

    const fetchRepos = async () => {
        const response = await fetch('/api/repos');
        repos = await response.json();
    };

    onMount(fetchRepos);
    onInterval(() => fetchRepos(), 30 * 1000);

    let showAddRepo = false;
    const close = () => showAddRepo = false;
    const open = () => showAddRepo = true;

    const handleNewRepo = async () => {
        await fetchRepos();
    }
</script>

<Tailwind/>
<div>
    <Fab onClick={open}/>
    <Error/>
    {#if showAddRepo}
        <AddNewRepo on:close={close} on:new-repo-added={handleNewRepo}/>
    {/if}

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

