# Gee WSGI Documentation Site

## Running the site locally

Make sure you are in the `site` directory. Using a Python virtual environment is recommended for local development, create one and activate it with

**Bash**
``` bash
python3 -m venv .venv
source .venv/bin/activate
```

**PowerShell**
``` powershell
python3 -m venv .venv
. .venv\Scripts\activate
```

Install dependencies to the virtual environment with

``` bash
pip install -r requirements.txt
```

I used Node to compile Tailwind which provides the CSS for the site. To go along with that, I added a script to the `package.json` for aiding in serving the site locally.

To serve the site locally, install the dependencies for compiling Tailwind

``` bash
npm install
```

then, using two terminals, run the following commands. In one terminal, run the Tailwind compiler

``` bash
npm run style
```

and in the other, start the Flask server

``` bash
npm run dev
```

This will serve the site in the browser at [localhost:5000](http://localhost:5000/).
