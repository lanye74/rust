const input = document.getElementById("input") as HTMLInputElement;
input.addEventListener("keyup", () => search(input.value));

const output = document.getElementById("output") as HTMLPreElement;

const championsText = await fetch("./champs.txt").then(res => res.text());
const champions = championsText.split("\r\n");



// i don't know what i'm doing. if i can't implement this in ts i have no hope for implementing it in rust
function search(query: string) {
	const queryCharacters = query.split("");

	let filtered: Map<string, number> = new Map();

	for(const [searchIndex, character] of Object.entries(queryCharacters)) {
		// if(character === "a") debugger;

		for(const [index, champion] of Object.entries(champions)) {
			const characterIndex = champion.toLowerCase().lastIndexOf(character);

			if(characterIndex === -1) {
				filtered.delete(index);
				continue;
			}

			if(!filtered.has(index) && searchIndex === "0") {
				filtered.set(index, characterIndex);
			} else {
				if(filtered.get(index)! >= characterIndex) {
					filtered.delete(index);
				} else {
					filtered.set(index, characterIndex);
				}
			}

		}
	}

	let filteredChampions = Array.from(filtered.keys());

	const outputArray = filteredChampions.map(champIndex => champions[parseInt(champIndex)]);

	output.textContent = outputArray.join("\n");
}



// needed to make this a "true" module
export {};
