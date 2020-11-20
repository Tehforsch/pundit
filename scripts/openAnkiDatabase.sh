testUser=ankiTesterBoy2
ankiPath=~/.local/share/Anki2/$testUser/
cp $1 $ankiPath/collection.anki2
anki -p $testUser
