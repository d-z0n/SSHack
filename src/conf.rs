use crate::theme::Theme;

#[derive(Clone)]
pub struct Conf {
    pub theme: Theme,
    pub banner: String,
}

impl Conf {
    pub fn get() -> Self {
        Self {
            theme: Theme::new("colors"),
            banner: r#"
         .           .                                    .,  G:      
        ;W          ;W  .    .                           ,Wt  E#,    :
       f#E         f#E  Di   Dt                 ..      i#D.  E#t  .GE
     .E#f        .E#f   E#i  E#i               ;W,     f#f    E#t j#K;
    iWW;        iWW;    E#t  E#t              j##,   .D#i     E#GK#f  
   L##Lffi     L##Lffi  E#t  E#t             G###,  :KW,      E##D.   
  tLLG##L     tLLG##L   E########f.        :E####,  t#f       E##Wi   
    ,W#i        ,W#i    E#j..K#j...       ;W#DG##,   ;#G      E#jL#D: 
   j#E.        j#E.     E#t  E#t         j###DW##,    :KE.    E#t ,K#j
 .D#j        .D#j       E#t  E#t        G##i,,G##,     .DW:   E#t   jD
,WK,        ,WK,        f#t  f#t      :K#K:   L##,       L#,  j#t     
EG.         EG.          ii   ii     ;##D.    L##,        jt   ,;     
,           ,                        ,,,      .,,                     
                                                                        
                "#
            .to_string(),
        }
    }
}
