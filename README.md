Progression (38/38) :
* ATTRIBUT
* ATTRIBUT_DE (fichier pas utilisé dans le code)
* ATTRIBUT_EN (fichier pas utilisé dans le code)
* ATTRIBUT_FR (fichier pas utilisé dans le code)
* ATTRIBUT_IT (fichier pas utilisé dans le code)
* BAHNHOF
* BETRIEB_DE
* BETRIEB_EN
* BETRIEB_FR
* BETRIEB_IT
* BFKOORD_LV95
* BFKOORD_WGS
* BFPRIOS
* BHFART (fichier pas utilisé dans le code)
* BHFART_60
* BITFELD
* DURCHBI
* ECKDATEN
* FEIERTAG
* FPLAN
* GLEIS
* GLEIS_LV95
* GLEIS_WGS
* GRENZHLT (fichier pas utilisé dans le code)
* INFOTEXT_DE
* INFOTEXT_EN
* INFOTEXT_FR
* INFOTEXT_IT
* KMINFO
* LINIE
* METABHF
* RICHTUNG
* UMSTEIGB
* UMSTEIGL
* UMSTEIGV
* UMSTEIGZ
* ZUGART
* ZEITVS (fichier pas utilisé dans le code)

TODO :
* Ajouter thiserror

Algorithme de calcul du trajet le plus court :
* Utilisation d'un arbre pour calculer les routes
* Seulement les lignes
* Maximum N changements
* Seulement les trajets possibles
* Pas de route sur 2 ou plusieurs jours
* Si une solution est trouvée, alors il faut arrêter certains path s'ils arrivent plus tard
* Prendre toutes les lignes différentes de la ligne actuel dans les 2 directions
* Prendre le trajet le plus tôt possible
* On ne revient pas sur ces pas
* Ne pas considérer les arrêts où les changements sont désactivés
* Utiliser une HashMap pour chaque arrêt contenant le temps le plus court pour l'atteindre, si un trajet arrive à un arrêt après le meilleur de temps => terminated


for loop + data HashMap : attribute
for loop + .last_mut()  : journey
.map().collect()        : bit_field, direction, holiday, through_service, transfer_administration, transfer_journey, transfer_line, stop

auto_increment          : holiday, journey, through_service, transfer_administration, transfer_journey, transfer_line
HashMap direct usage    : stop
pk_type_converter       : direction, journey
