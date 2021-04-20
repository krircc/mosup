<template>
  <div id="home">
      <main>
        <div id="home-top"></div>
        <div id="hcontainer">
          <div id="header">
            <li  ><router-link to="#">推荐</router-link></li>
            <li  id="item"><router-link to="/">关注</router-link></li>
            <li  id="item"><router-link to="#">最热</router-link></li>
            <!-- <li  id="item"><router-link to="/best">推荐</router-link></li>
            <li  id="item"><router-link to="/hot">最热</router-link></li>
            <li  id="item"><router-link to="/top">Top</router-link></li>
            <li  id="item"><router-link to="/care">Care</router-link></li>
            <li  id="item"><router-link to="/rise">Rise</router-link></li> -->
          </div>
        </div>
        <div id="container">
          <div id="center">
              <div id="content">
                  <div id="theme">
                        <div id="items" v-for="(theme, index) in theme_office_list" :key="index">
                            <div id="office">
                                <div id="avatar">
                                  <a :href="'/a/user/' + theme.user_id"><img :src= theme.user_avatar ></a>
                                </div>
                                <div id="mobile">
                                  <div id="office-title">
                                    <span id="community-name"><a :href="'/a/community/' + theme.community_name">{{theme.community_name_cn}}</a></span>
                                    <span id="title-name"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id" title="theme.title">{{theme.title}}</a></span>
                                  </div> 
                                  <div id="detail">
                                      <div class="info" id="community_tag_name"><a :href="'/a/' + 'community/' + theme.community_name + '/' + theme.community_tag_name">{{theme.tag_name_cn}}</a></div>
                                      <div class="info" id="comment"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id">{{theme.comment_count}}</a></div>
                                      <div class="info" id="view">{{theme.view_count}}</div>
                                      <div class="info" id="time">{{theme.rtime}}</div>
                                  </div>
                                </div> 
                              </div>
                            </div>
                        </div>
                  <infinite-scroll :parent-element="true" :infinite-function = "infinite"  :infinite-loading = "infiniteLoading" >
                      <div slot>
                            <div id="theme">
                              <div id="items" v-for="(theme, index) in theme_list" :key="index">
                                  <div id="item">
                                      <div id="avatar">
                                        <a :href="'/a/user/' + theme.user_id"><img :src= theme.user_avatar ></a>
                                      </div>
                                      <div id="mobile">  
                                          <div id="item-title">
                                            <span id="community-name"><a :href="'/a/community/' + theme.community_name">{{theme.community_name_cn}}</a></span>
                                            <span id="title-name"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id" title="theme.title">{{theme.title}}</a></span>
                                          </div> 
                                          <div id="detail">
                                              <div class="info" id="community_tag_name"><a :href="'/a/'+ 'community/' + theme.community_name + '/'  + theme.community_tag_name">{{theme.tag_name_cn}}</a></div>
                                              <div class="info" id="comment"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id">{{theme.comment_count}}</a></div>
                                              <div class="info" id="view">{{theme.view_count}}</div>
                                              <div class="info" id="time">{{theme.rtime}}</div>
                                          </div> 
                                      </div>
                                  </div>
                              </div>
                            </div>
                      </div>
                      <div id="loading" slot="bottom">
                          <h3 style="color:gray; margin: 0.5rem; text-align:center;">加载中...</h3>
                      </div>
                  </infinite-scroll>
              </div>
          </div>
          <div id="aside">
              <side></side>
          </div>
          <gotop></gotop>
        </div>
      </main>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Gotop from '../../components/gotop/Gotop.vue'
import Side from '../../components/side/Side.vue'
import InfiniteScroll from '../../components/infiniteScroll/InfiniteScroll.vue'
export default {
  name: 'home',
  components: {
      "infinite-scroll": InfiniteScroll,
      "gotop": Gotop,
      "side": Side
  },
  data: function() {
    return {
      theme_list: [],
      signin_user: '',
      theme_office_list: [],
      infiniteLoading: false,
      theme_page_count: '',
      page_id: 1
    }
  },
  mounted: function() {
      if (JSON.parse(localStorage.getItem('signin_user'))) {
              this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
              let signin_user_id = JSON.parse(localStorage.getItem('signin_user')).id
              let data = { 
                  user_id: signin_user_id,
                  page_id: this.page_id
              }
              fetch(URLprefix + 'api/theme_list',{
                    body: JSON.stringify(data), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.theme_page_count = json.theme_page_count
                  json.theme_list.map((item) => {
                    if (item.user_avatar == "") {
                      item.user_avatar = "https://www.gravatar.com/avatar/1"
                    }
                  })
                  json.theme_list.filter((item) => { 
                        if ((item.community_name === 'ouisrc') && (item.community_tag_name === 'Announce')) {
                            this.theme_office_list.push(item)
                        }else{
                            this.theme_list.push(item)
                        }
                  }) 
              }).catch((e) => {console.log(e)})
      }else{
              let data = { 
                    user_id: 0,
                    page_id: this.page_id
              }
              fetch(URLprefix + 'api/theme_list',{
                    body: JSON.stringify(data), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
              }).then(response => response.json()).then(json => {
                
                    this.theme_page_count = json.theme_page_count
                    json.theme_list.map((item) => {
                      if (item.user_avatar == "") {
                        item.user_avatar = "https://www.gravatar.com/avatar/1"
                      }
                    })  
                    json.theme_list.filter((item) => { 
                        if ((item.community_name === 'ouisrc') && (item.community_tag_name === 'Announce')) {
                            this.theme_office_list.push(item)
                        }else{
                            this.theme_list.push(item)
                        }
                    })       
                    // console.log(this.theme_office_list)
                    // console.log(this.theme_list)    
              }).catch((e) => { console.log(e) })
      }
  },
  methods: {
        infinite() {
            if(this.infiniteLoading){
                return false;
            }
            this.infiniteLoading = true;

            if (this.page_id < this.theme_page_count) {
                let page_id =  this.page_id + 1
                this.page_id = page_id

                setTimeout(()=> {
                  if (JSON.parse(localStorage.getItem('signin_user'))) {
                      this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
                      let signin_user_id = JSON.parse(localStorage.getItem('signin_user')).id
                      let data = { 
                        user_id: signin_user_id,
                        page_id: page_id
                      }
                      fetch(URLprefix + 'api/theme_list',{
                          body: JSON.stringify(data),
                          headers: {
                            'content-type': 'application/json'
                          },
                          method: 'POST',
                          mode: 'cors'
                      }).then(response => response.json())
                      .then(json => {
                          json.theme_list.map((item) => this.theme_list.push(item))
                      }).catch((e) => {console.log(e)})
                  }else{
                      let data = { 
                        user_id: 0,
                        page_id: page_id
                      }
                      fetch(URLprefix + 'api/theme_list',{
                          body: JSON.stringify(data),
                          headers: {
                            'content-type': 'application/json'
                          },
                          method: 'POST',
                          mode: 'cors'
                      }).then(response => response.json())
                      .then(json => {
                          json.theme_list.map((item) => this.theme_list.push(item))
                      })
                      .catch((e) => {
                        console.log(e)
                      })
                  }
                  this.infiniteLoading = false;
                  if(this.theme_list.length > ((this.theme_page_count -1)*22) ){
                        this.infiniteLoading = true;
                        document.getElementById("loading").style.display = "none"
                  }console.log(this.count)
                }, 400);   
            }else {
                this.infiniteLoading = false;
                document.getElementById("loading").style.display = "none"
            }
        }
    }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#hcontainer {
  width: 100%;
  position: fixed;
  background-color: #ffffff;
  border-bottom: 1px solid #ecf0f0;
}
#header {
  line-height: 26px;
  margin-top: 0.5rem;
}
#header li {
  display: inline-block;
  /* font-weight: bold; */
  font-size: 0.9rem;
  /* color: #0a0a0ad2; */
  /* text-shadow: gray 0 1px 0;
  text-shadow: 0 0 20px gray; */
  /* text-shadow: 0 0 5px #fff, 0 0 44px #fff, 0 0 5px khaki, 0 0 5px blue, 0 0 5px green; */
   /* text-shadow:0 0 6px #F96, -1px -1px  #FFF, 1px -1px  #444; */
   /* text-shadow: 1px 1px 0 #f96,-1px -1px 0 #f96;  */
   text-shadow: 0.2px 0.2px 0.2px gray;
}
#container #center #content #theme {
    border: 1px solid #ecf0f0;
}
#center #items #office, #center #items #item {
    display: flex;
    border-bottom: 1px solid #f3e1f8;
    /* border: 1px solid #ecf0f0; */
}
#container #center #content #theme #items #office, #container #center #content #theme #items #item {
    display: flex;
}
#center #content #office #office-title a {
    color: #b93bf3;
    font-weight: bold;
}
#center #content #office-title #title-name, #center #content #item-title #title-name {
    padding-left: 0.3rem;
}
#center #item #item-title #title-name a:visited {
    color: gray;
}
#center #theme #items #community-name {
    border-radius: 10%;
    color: #1362f5;
    font-size: 0.8rem;
    background-color: #dddddd;
    padding: 0.2rem;
}
@media only screen and (max-width: 767px) {
    #home-top {
        padding-top: 3rem;
    }
    main{
        margin: 0 auto;
        width: 98%;
    }
    #header {
        padding: 0.5rem 0.3rem ;
    }
    #header #item {
      margin: 0 1rem;
    }
    #container {
      padding-top: 3.3rem;
      margin-bottom:0.5rem;
    }
    #container #center {
        margin-bottom: 1rem;
    }
    #container #center #theme #avatar img {
        width: 2.3rem;
        height: 2.3rem;
        margin: 0.2rem;
        vertical-align: middle;
        border-radius: 50%;
    }
    #center #items #office #office-title, #center #items #item #item-title {
        padding: 0.4rem 0.3rem 0.1rem 0.1rem;
        flex: 1;
    }
    #center #theme #items #detail {
        display: flex;
        color:gray;
        margin-bottom: 0.2rem;
        font-size: 0.88rem;
    }
    #center #items #detail #community_tag_name {
        margin-left: 0.3rem;
        color: #9d5cf1;
    }
    #center #items #detail #comment,#center #items #detail #view,#center #items #detail #time {
        margin-left: 1rem;
        padding-top: 0.3rem;
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    #home-top {
      padding-top: 3rem;
    }
    main{
        margin: 0 auto;
    }
    #header{
        padding-top: 0.4rem;
        margin: 0 auto;
        width: 77%;
    }
    #header #item {
      margin: 0.6rem 1rem;
    }
    #container {
      width: 77%;
      margin: 4rem auto 1rem;
    }
    #container #center {
        margin-bottom: 1rem;
    }
    #center #content #theme #items #office #mobile, #center #content #theme #items #item #mobile {
        display: flex;
        flex: 1;
    }
    #container #center #theme #items #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin-top: 0.2rem;
        border-radius: 50%;
    }
    #center #items #office #office-title, #center #items #item #item-title {
        padding: 0.7rem 0.2rem;
        width: 70%;
    }
    #center #items #detail {
        flex: 1;
        display: flex;
        padding-top: 1rem;
        color:gray;
        font-size: 0.88rem;
    }
    #center #items #detail #community_tag_name {
        text-align: center;
        flex-basis: 3rem;
        color: #9d5cf1;
    }
    #center #items #detail #comment {
        text-align: center;
        flex-basis: 2rem;
        color: #9d5cf1;
    }
    #center #items #detail #view {
        text-align: center;
        flex-basis: 3rem;
    }
    #center #items #detail #time {
        text-align: center;
        flex-grow:1;
    }
}
@media only screen and (min-width: 1000px) and (max-width: 1399px) {
    #home-top {
      padding-top: 3rem;
    }
    main{
        margin: 0 auto;
    }
    #header {
        padding-top: 0.4rem;
        margin: 0 auto;
        width: 80%;
    }
    #header #item {
      margin: 0.6rem 1rem;
    }
    #container {
      width: 80%;
      margin: 4rem auto 2rem; 
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 76%;
        margin-right: 0.8rem;
    }
    #center #content #theme #items #office #mobile, #center #content #theme #items #item #mobile {
        display: flex;
        flex: 1;
    }
    #container #center #theme #items #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin-top: 0.2rem;
        border-radius: 50%;
    }
    #center #items #office #office-title, #center #items #item #item-title {
        padding: 0.7rem 0.2rem;
        width: 66%;
    }
    #center #items #detail {
        flex: 1;
        display: flex;
        padding-top: 1rem;
        color:gray;
        font-size: 0.88rem;
    }
    #center #items #detail #community_tag_name {
        text-align: center;
        flex-basis: 3rem;
        color: #9d5cf1;
    }
    #center #items #detail #comment {
        text-align: center;
        flex-basis: 2rem;
        color: #9d5cf1;
    }
    #center #items #detail #view {
        text-align: center;
        flex-basis: 3rem;
    }
    #center #items #detail #time {
        text-align: center;
        flex-grow:1;
    }
    #container #aside {
        flex: 1;
    }
}
@media only screen and (min-width: 1400px) {
    #home-top {
      padding-top: 3rem;
    }
    main{
        margin: 0 auto;
    }
    #header {
        padding-top: 0.4rem;
        margin: 0 auto;
        width: 1040px;
    }
    #header #item {
      margin: 0.6rem 1rem;
    }
    #container {
      margin: 4rem auto 2rem; 
      width: 1040px;
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 76%;
        margin-right: 0.8rem;
    }
    #center #content #theme #items #office #mobile, #center #content #theme #items #item #mobile {
        display: flex;
        flex: 1;
    }
    #container #center #theme #items #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin-top: 0.2rem;
        border-radius: 50%;
    }
    #center #items #office #office-title, #center #items #item #item-title {
        padding: 0.7rem 0.2rem;
        width: 73%;
    }
    #center #theme #items #detail {
        flex: 1;
        display: flex;
        padding: 1rem 0.1rem 0 0;
        color:gray;
        font-size: 0.88rem;
    }
    #center #items #detail #community_tag_name {
        text-align: center;
        flex-basis: 3rem;
        color: #9d5cf1;
    }
    #center #items #detail #comment {
        text-align: center;
        flex-basis: 2rem;
        color: #9d5cf1;
    }
    #center #items #detail #view {
        text-align: center;
        flex-basis: 3rem;
    }
    #center #items #detail #time {
        text-align: center;
        flex-grow:1;
    }
    #container #aside {
        flex: 1;
    }
}
        
</style>