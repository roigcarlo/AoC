Kowalsky = '''
                   im
               portSreNimportS
              sysNimportSnu
             mpySasSnpNwit
             hSopen(sys.ar
              gv[1])SasSdat
             a:NTventsS=Snp.
            array(re.findall('
           (\d+),(\d+)S->S(\d+),
         (\d+)',Sdata.read()),Sdty
        pe='int')NTfieldS=Snp.zeros(
       shape=(np.max(vents)S+S1,Snp.ma
      x(vents)S+S1))NTforSvSinSvents:NTT
    ifSv[0]S==Sv[2]SorSSv[1]S==Sv[3]:NTTT
   field  [min(v[0],v[2]):max(v[0],v[2])+1
  ,mi     n(v[1],v[3]):max(v[1],v[3]    )+1
  ]S     +=S1NTTelse:NTTTfield[(np.a     rra
 y      (list(range(v[0],v[2]+(1,-1)      [i
        nt(v[0]>v[2])],(1,-1)[int(v[0       ]
       >v[2])]))),np.array(list(range       (
        v[1],v[3]+(1,-1)[int(v[1]>v[3         
        ])],(1,-1)[int(v[1]>v[3])]))
         ))]S+=S1NTprint("Kowalsky,S
          reportSgeothermalSactivi
           ty!:"    ,Snp.count_n
           onze              ro(
       field[fie             ld>1]))N
'''

exec(Kowalsky.replace('\n','').replace(' ','').replace('S',' ').replace('T','\t').replace('N','\n'))