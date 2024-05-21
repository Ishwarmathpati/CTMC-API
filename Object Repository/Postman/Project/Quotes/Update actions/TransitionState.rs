<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TransitionState</name>
   <tag></tag>
   <elementGuidId>cefb4339-2272-4f45-ac0e-3975ddaad04d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${quote-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;transitionState\&quot;,\n            \&quot;state\&quot; : {\n              \&quot;typeId\&quot; : \&quot;state\&quot;,\n              \&quot;id\&quot; : \&quot;${state-id}\&quot;\n            }\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>4a471bfa-8318-4487-9a77-e353848441fb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c8bfc221-2f05-441c-84c2-306e263a9755</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/quotes/${quote-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>c4b780d8-ae05-45f6-aa23-251d552a10a3</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>0a3834da-1e5a-4cd5-bb34-42cf9fde44b4</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.quote-id</defaultValue>
      <description></description>
      <id>4f52010f-9ca1-4c6b-93bf-2b91ffd9729f</id>
      <masked>false</masked>
      <name>quote-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.quote-version</defaultValue>
      <description></description>
      <id>fa580e56-bd37-4fed-8974-78c81c192e76</id>
      <masked>false</masked>
      <name>quote-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.state-id</defaultValue>
      <description></description>
      <id>ad68ebbd-f823-444d-b94b-2f78cd976f05</id>
      <masked>false</masked>
      <name>state-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
