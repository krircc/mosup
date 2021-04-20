<template>
    <div id="gotop">
        <main>
            <button @click="topFunction" id="myBtn" title="Go to top">Top</button>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'gotop',
  mounted: function() {
      window.addEventListener('scroll',function(){
        var scrollTop = document.documentElement.scrollTop || document.body.scrollTop;
        if(scrollTop>20){
            document.getElementById("myBtn").style.display = "block";
        }else{
            document.getElementById("myBtn").style.display = "none";
        }
      },true)
  },
  methods: {
      topFunction() {
        // When the user clicks on the button, scroll to the top of the document
        document.body.scrollTop = 0; // For Chrome, Safari and Opera 
        document.documentElement.scrollTop = 0; // For IE and Firefox
      }
  }
}
</script>

<style scoped>
@media only screen and (max-width: 767px) {
    main #myBtn {
        display: none; /* Hidden by default */
        position: fixed; /* Fixed/sticky position */
        bottom: 1rem; /* Place the button at the bottom of the page */
        right: 6%; /* Place the button 30px from the right */
        z-index: 99; /* Make sure it does not overlap */
        border: 1px solid #ecf0f0; /* Remove borders */
        outline: none; /* Remove outline */
        background-color: #ecf0f0; /* Set a background color */
        font-weight: bold;
        color: black; /* Text color */
        cursor: pointer; /* Add a mouse pointer on hover */
        padding: 8px; /* Some padding */
        border-radius: 10%; /* Rounded corners */
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    main #myBtn {
        display: none; /* Hidden by default */
        position: fixed; /* Fixed/sticky position */
        bottom: 1rem; /* Place the button at the bottom of the page */
        right: 15%; /* Place the button 30px from the right */
        z-index: 99; /* Make sure it does not overlap */
        border: 1px solid #ecf0f0; /* Remove borders */
        outline: none; /* Remove outline */
        background-color: #ecf0f0; /* Set a background color */
        font-weight: bold;
        color: black; /* Text color */
        cursor: pointer; /* Add a mouse pointer on hover */
        padding: 8px; /* Some padding */
        border-radius: 10%; /* Rounded corners */
    }
}
@media only screen and (min-width: 1000px) {
    main #myBtn {
        display: none; /* Hidden by default */
        position: fixed; /* Fixed/sticky position */
        bottom: 1rem; /* Place the button at the bottom of the page */
        right: 22%; /* Place the button 30px from the right */
        z-index: 99; /* Make sure it does not overlap */
        border: 1px solid #ecf0f0; /* Remove borders */
        outline: none; /* Remove outline */
        background-color: #ecf0f0; /* Set a background color */
        font-weight: bold;
        color: black; /* Text color */
        cursor: pointer; /* Add a mouse pointer on hover */
        padding: 8px; /* Some padding */
        border-radius: 10%; /* Rounded corners */
    }
}
</style>