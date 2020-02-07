<script>
    import AddRepo from "../modals/AddRepo.svelte";
    import OnEscape from "../support/OnEscape.svelte";

    let modalOpen = false


    const close = () => {
        modalOpen = false
    }
</script>

<OnEscape action={close} />

<div class="container">
    <input type="checkbox" id="the-fab"
           bind:checked={modalOpen}
           class="text-white fill-current rounded-full bg-blue-600 shadow-xl hover:shadow-2xl hover:bg-blue-700"/>
    <label class="icon bg-blue-600" for="the-fab">
    </label>
    <nav class="nav">
        <AddRepo on:new-repo-added on:close={close} />
    </nav>
</div>

<style>
    .container, .icon, .nav {
        position: absolute;
    }

    .container {
        top: 4rem;
        left: 1rem;
    }

    #the-fab {
        display: none;
    }

    #the-fab:checked ~ .nav {
        opacity: 1;
        transform: translateX(15%);
    }

    #the-fab:checked ~ .icon {
        transform: rotate(135deg);
        box-shadow: 0 0 0 0 transparent;
    }


    .icon {
        z-index: 999;
        width: 43px;
        height: 43px;
        border-radius: 100%;
        transition: all 0.5s ease-in-out;
        box-shadow: 1px 3px 10px 0 rgba(0, 0, 0, 0.3);
        cursor: pointer;
    }

    .icon:before {
        position: absolute;
        top: 20px;
        left: 9px;
        content: '';
        width: 25px;
        height: 2px;
        background: #FFF;
        transform: rotate(90deg);
    }

    .icon:after {
        position: absolute;
        top: 20px;
        left: 9px;
        content: '';
        width: 25px;
        height: 2px;
        background: #FFF;
    }

    .nav {
        opacity: 0;
        transition: all 0.5s ease-in-out;
        background: #FFF;
        border-radius: 5px;
        box-shadow: 2px 3px 10px 0 rgba(0, 0, 0, 0.1);
    }
</style>
