/** @type {import('tailwindcss').Config} */
module.exports = {
  safelist: [
    "bg-[url('images/responsible-computing.jpg')]",
    "bg-[url('images/health-and-wellness.jpg')]",
    "bg-[url('images/environmental-impact.jpg')]",
    "bg-[url('images/educational-reform.jpg')]",
    "bg-[url('images/financial-freedom.jpg')]",
    "bg-[url('images/luggage.jpg')]",
    "bg-[url('images/gumby.svg')]",
    "bg-[url('images/passion-fruit.png')]",
    "bg-[url('images/blue-eel.jpg')]",
    "bg-[url('images/fire.jpg')]",
    "bg-[url('images/space-exploration.jpg')]",
    "bg-[url('images/fitness.jpg')]",
    "bg-[url('images/natural-beauty.jpg')]",
    "bg-[url('images/physics-and-math.jpg')]",
    "bg-[url('images/robotics-and-automation.jpg')]",
  ],
  content: {
    files: ["./index.html", "../app/src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
}