from deep_underwater import encyclopedia_of_the_seas

crabs_go_backwards = '''
                                   (lǝnɟ‾uᴉɯ ',:spɹɐʍʞɔɐq oƃ sqɐɹɔ 'ʎʞslɐʍoʞ,)ʇuᴉɹd    
    
                            (pǝʇɐlnɔlɐɔ‾ʎlƃuᴉɹoq‾uᴉɯ 'lǝnɟ‾uᴉɯ)uᴉɯ = lǝnɟ‾uᴉɯ            
([sqɐɹɔ uᴉ u ɹoɟ ᄅ/((Ɩ+(ᴉ-u)sqɐ)*(ᴉ-u)sqɐ)])ɯns˙du = pǝʇɐlnɔlɐɔ‾ʎlƃuᴉɹoq‾uᴉɯ            
                                                      :((sqɐɹɔ)uǝl)ǝƃuɐɹ uᴉ ᴉ ɹoɟ        
       
                                                           ǝzᴉsxɐɯ˙sʎs = lǝnɟ‾uᴉɯ        
                    (,ʇuᴉ,=ǝdʎʇp '(()pɐǝɹ˙ɐʇɐp ',+p\,)llɐpuᴉɟ˙ǝɹ)ʎɐɹɹɐ˙du = sqɐɹɔ        
                                                      :ɐʇɐp sɐ ([Ɩ]ʌƃɹɐ˙sʎs)uǝdo ɥʇᴉʍ    
                                                             :,‾‾uᴉɐɯ‾‾, == ‾‾ǝɯɐu‾‾ ɟᴉ
       
                                                                       du sɐ ʎdɯnu ʇɹodɯᴉ
                                                                               sʎs ʇɹodɯᴉ
                                                                                ǝɹ ʇɹodɯᴉ
'''

crab_to_snake = '\n'.join([line[::-1] for line in crabs_go_backwards.translate(crabs_go_backwards.maketrans(encyclopedia_of_the_seas)).split('\n')[::-1]])

exec(crab_to_snake)