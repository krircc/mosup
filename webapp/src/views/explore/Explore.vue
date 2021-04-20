<template>
  <div  id="explore">
      <!-- <mnav id="mnav"></mnav> -->
      <main>
        <div id="container">
          <div id="center">
              <div id="community">
                    <div id="title">新发现 </div>
                    <P>探索-互动：探区：探友活跃之地. 探识：探友成就值.</P>
                    <div v-for="(community_category_cn, index) in community_category_list" :key="index">
                      <div id="outer">
                          <div id="community_category" >{{ community_category_cn }}</div>
                          <div v-for="(community, index) in communitys" :key="index">
                            <div v-if="community.community_category_cn == community_category_cn">
                              <div id="inner">
                                  <div id="detail">
                                      <span class="one"><a   :href="'/a/community/' + community.community_name"><img style="width:2.5rem;height:2.5rem;margin: 0 8px -4px 0;" :src="community.community_img"/></a> </span>
                                      <span class="one" id="full">
                                          <div id="community_name" > 
                                            <span id="item">
                                                  <a  :href="'/a/community/' + community.community_name">
                                                      {{ community.community_name_cn }}
                                                  </a>
                                              </span>
                                              <span id="item-right" style="float: right;" >
                                                    <button v-if="signin_user == ''" id="join" @click="join"><strong style="color: green; font-size: 1rem" >+</strong>加入</button>
                                                    <button v-else-if="(joinor == false) && signin_user.id" id="join" @click="join"><strong style="color: green; font-size: 1rem" >+</strong>加入</button>
                                                    <button v-else id="joined">已加入</button>
                                              </span>
                                          </div>
                                          <div id="infos">
                                              <span id="info"><a :href="'/a/user/' + community.user_id"> {{ community.user_id }} </a></span> • 
                                              <span id="info"><a href="#"> {{ community.community_category_cn }} </a></span> •  
                                              <!-- <span id="info"><a :href="'/a/community/' + community.community_category"> {{ community.community_category_cn }} </a></span> •   -->
                                              <span id="info"> {{ community.created_at }}</span>
                                          </div>
                                      </span>
                                  </div>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
              </div>
          </div>
          <div id="aside">
                  <side></side>
          </div>
        </div>
      </main>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Mnav from '../../components/nav/Mnav.vue'
import Side from '../../components/side/Side.vue'
export default {
  name: 'explore',
  components: {
    "mnav": Mnav,
    "side": Side
  },
  data: function() {
    return {
      signin_user:'',
      communitys:'',
      community_category_list: [],
      joinor: false
    }
  },
  mounted: function() {
      if (JSON.parse(localStorage.getItem('signin_user'))) {
            this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
      }
      fetch(URLprefix + 'api/communitys',{
          method: 'GET',
      }).then(response => response.json())
      .then(json => {
             json.communitys.map((item) => {
                item.created_at = item.created_at.slice(0,10)
            })
            this.communitys = json.communitys
            // console.log(json.communitys)
            let community_category_result = {}
            for (let index = 0; index < json.communitys.length; index++) {
              // console.log(json.communitys[index].community_category_cn)
              community_category_result[json.communitys[index].community_category_cn] = json.communitys[index].community_category_cn
            }
            
            for(var key in community_category_result){ 
                this.community_category_list.push(key); 
            } 
            // console.log(community_category_list)
      }).catch((e) => {
        console.log(e)
      })
  },
  methods: {
    join(e) {
        if (localStorage.getItem('signin_user')){
            let join = document.getElementById("join")
            join.style.color = "#02ca13"
            join.innerHTML = "已加入"
            let user_id = JSON.parse(localStorage.getItem('signin_user')).id
            let community_name = e.target.parentNode.previousElementSibling.children[0].pathname.slice(13)
            let data = { 
                user_id: user_id,
                community_name: community_name
            }
            fetch(URLprefix + 'api/community_join', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
            }).then(response => response.json()).then(json => {
                  //  window.location.reload ( true )
              }).catch((e) => {
                console.log(e)
              })
        }else{
            alert("请先登录.")
        }
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#container {
    margin-bottom: 1rem;
}
main #center {
    border: 1px solid #ecf0f0;
    padding: 0.5rem;
}
main #center #title {
  font-size: 1.1rem;
  font-weight: bold;
  margin-bottom: 0.5rem;
}
#center #community #community_category {
    font-weight: bold;
    font-size: 0.9rem;
    margin: 1rem 0;
    padding: 5px 0;
}
#center #community #detail {
    margin: 0.5rem;
    /* background-color: #f2f3f3; */
    border: 1px solid #f2f3f3;
}
#center #community .one {
         display: inline-block;
}
#center #community #detail #community_name #item {
    padding: 0.3rem 0.3rem 0.3rem 0;
    /* background-color: #cbdffd; */
}
#center #community #detail #community_name a {
    font-weight: bold;
    font-size: 0.9rem;
    color: #4f3d8d;
}
#center #community #detail #community_name #item-right button {
    height:1.4rem;
    font-size: 0.8rem; 
    border: none; 
    background-color:#cbdffd;
}
#center #community #detail #infos{
    margin-top: 2px;
}
#center #community #detail #info{
    display: inline-block;
    font-size: 14px;
}
@media only screen and (max-width: 767px) {
    main  {
      padding-top: 3.5rem;
      margin: 0 auto;
      width: 96%;
    }
    main #center  {
      margin-bottom: 1rem;
    }
    #center #community #detail #full {
        width: 82%;
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    main {
      padding-top: 4rem;
      margin: 0 auto;
      width: 72%;
    }
    main #center  {
      margin-bottom: 1rem;
    }
    #center #community #detail #full {
        width: 90%;
    }
}
@media only screen and (min-width: 1000px) {
    main {
        margin: 0 auto;
        width: 1040px;
        padding-top: 4.4rem;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 0.8rem;
    }
    #container #aside {
        flex: 1;
    }
    #center #community #outer {
        display: inline-block;
    }
    #center #community #inner {
      margin-top: -1rem;
        width: 48rem;
    }
    #center #community #detail {
        float: left;
        display: block;
        width: 22rem;
    }
    #center #community #detail #full {
        width: 18rem;
    }
}
</style>