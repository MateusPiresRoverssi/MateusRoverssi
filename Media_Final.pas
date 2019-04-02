program Media_Final;
   var
     A1,A2,P1,Ma1,Mb1,P2,Ma2,Mb2,MF,QtdAulas,QtdFaltas,Frequencia:real;
begin
   readln(P1,Ma1,Mb1,P2,Ma2,Mb2,QtdAulas,QtdFaltas);
   A1:=(P1*0.7)+(Ma1*0.2)+(Mb1*0.1);
   A2:=(P2*0.7)+(Ma2*0.2)+(Mb2*0.1);
   MF:=(A1+(2*A2))/3;
   writeln('Média Final = ',MF:0:1);
   Frequencia:=(QtdFaltas*100) / QtdAulas;
   writeln('Frequencia = ',Frequencia:0:0,'%');

   if (((MF >= 5.0) or (MF >= 3.0)) and (Frequencia >= 75))Then
    writeln('APROVADO')

   else if (((MF < 5.0) and (MF >=3.0)) and (Frequencia >=75)) Then
writeln('RECUPERACAO')
   else if ((MF<3.0) and (Frequencia >=75)) Then
   writeln('REPROVADO POR NOTA')
   else if (Frequencia < 75) Then
   writeln('REPROVADO POR FALTA')
end.
