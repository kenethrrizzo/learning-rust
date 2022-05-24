const invoke = window.__TAURI__.invoke

const btn = document.querySelector('#btn')
const btnCity = document.querySelector('#btnCity')
const city = document.querySelector('#city')
const showUser = document.querySelector('#showUser')

btn.addEventListener('click', () => {
    alert('Hello, World')
})

btnCity.addEventListener('click', () => {
    alert(city.value)
    invoke('show_city', {
        cityName: city.value
    })
})

showUser.addEventListener('click', () => {
    invoke('hello_user', {
        username: 'Keneth'
    })
    .then(msg => console.log(msg))
    .catch(err => console.error(err))
})
