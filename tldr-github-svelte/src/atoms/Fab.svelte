<script>
    import GithubIcon from "./GithubIcon.svelte";

    export let onClick
</script>

<button class="fab" on:click={onClick}>
    <GithubIcon icon="plus" />
</button>

<style>
    .fab {
        background-color: #3E6DB2;
        color: white;
        z-index: 999;
        border-radius: 100%;
        height: 50px;
        width: 50px;
        position: fixed;
        top: 2rem;
        left: 1rem;

        fill: white;
    }

    button {
        box-shadow: 0 3px 6px rgba(0,0,0,0.16), 0 3px 6px rgba(0,0,0,0.23);
    }

    button:hover {
        box-shadow: 0 3px 6px rgba(0,0,0,0.24), 0 3px 12px rgba(0,0,0,0.36);
    }

</style>