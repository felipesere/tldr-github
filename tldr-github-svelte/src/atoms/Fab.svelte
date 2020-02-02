<script>
    import GithubIcon from "./GithubIcon.svelte";

    export let onClick
</script>

<button class="fab text-white fill-current z-10 rounded-full bg-blue-600 shadow-xl hover:shadow-2xl hover:bg-blue-700" on:click={onClick}>
    <GithubIcon icon="plus" />
</button>

<style>
    .fab {
        height: 50px;
        width: 50px;
        position: fixed;
        top: 2rem;
        left: 1rem;
    }
</style>