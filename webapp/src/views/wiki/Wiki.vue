<template>
  <div  id="wiki">
      <main>
        <div id="container">
          <wikiside id="wikiside"></wikiside>
          <div id="content">
              <p><strong>Wiki</strong> This is the wiki page.</p> <br>
          </div>
        </div>
      </main>
  </div>
</template>

<script>
/* eslint-disable */
import Wikiside from '../../components/wikiside/Wikiside.vue'
export default {
  name: 'wiki',
  components: {
    "wikiside": Wikiside
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px;
    }
}
@media only screen and (min-width: 1000px) {
    main {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #content {
        width: 80%;
        margin-left: 1rem;
    }
    #container #wikiside {
        flex: 1;
        background-color: #f1ebeb;
    }
}
</style>