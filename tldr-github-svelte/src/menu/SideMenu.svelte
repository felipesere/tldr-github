<script>
    import GithubIcon from "../atoms/GithubIcon.svelte";
    export let onClickAdd
</script>

<div class="side-nav stack">
    <button type="button" class="nav-button" on:click={onClickAdd}>
        <GithubIcon icon="plus" />
    </button>
    <button type="button" class="nav-button">
        <GithubIcon icon="sync" />
    </button>
</div>

<style>
    .side-nav {
        flex: none;
        background-color: #2C5282;
        width: 4rem;
        padding-top: 4rem;
    }

    .nav-button {
        align-items: center;
        background: white;
        border-radius: .5rem;
        color: black;
        display: flex;
        height: 2.5rem;
        justify-content: center;
        margin-left: auto;
        margin-right: auto;
        margin-bottom: 1rem;;
        opacity: 0.5;
        overflow: hidden;
        padding: 5px;
        width: 2.5rem;
    }

    .nav-button:hover {
        box-shadow: 0 0 5px black;
        opacity: 1;
    }
</style>